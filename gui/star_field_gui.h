#pragma once

#include <QtDataVisualization/qscatterdataproxy.h>
#include <QtDataVisualization/qvalue3daxis.h>
#include <QtDataVisualization/q3dscene.h>
#include <QtDataVisualization/q3dcamera.h>
#include <QtDataVisualization/qscatter3dseries.h>
#include <QtDataVisualization/q3dtheme.h>
#include <QtDataVisualization/QCustom3DItem>
#include <QtDataVisualization/Q3DScatter>
#include <QtCore/qmath.h>
#include <QTimer>

#include <compute>

using namespace QtDataVisualization;
namespace mgs
{
  class StarFieldGUI : public QObject
  {
    Q_OBJECT
    
  public:
    explicit StarFieldGUI(Q3DScatter *scatter);
    ~StarFieldGUI();
    
    void updateFieldState();
    void clearField();
    void generateField();
                       
                       
  public Q_SLOTS:
    void setFieldLines(int lines);
    void setArrowsPerLine(int arrows);
    void toggleRotation();
    void triggerRotation();
    void toggleSun();

    void sl_make_polygon(int stars);

    void sl_make_tetrahedron();
    void sl_make_octahedron();
    void sl_make_hexahedron();
    void sl_make_dodecahedron();
    void sl_make_icosahedron();

  private:
    Q3DScatter *m_graph;

    QTimer m_rotationTimer;

    int m_fieldLines;
    int m_arrowsPerLine;
    int m_freePointMassCube;
    
    QScatter3DSeries *m_freePointMass;
    QScatter3DSeries *m_stars;
    QCustom3DItem *m_sun;

    std::vector<Star> c_stars;
    
    QScatterDataArray *m_freePointMassArray;
    QScatterDataArray *m_starArray;

    float m_angleOffset;
    float m_angleStep;
  };
}